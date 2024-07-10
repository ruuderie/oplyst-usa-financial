import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenuGroup from '../components/ui/dropdown-menu/DropdownMenuGroup.vue';

const meta = {
  title: 'DropdownMenuGroup',
  component: DropdownMenuGroup,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenuGroup>;

export default meta;
type Story = StoryObj<typeof DropdownMenuGroup>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};