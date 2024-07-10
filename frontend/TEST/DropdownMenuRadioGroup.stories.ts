import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenuRadioGroup from '../components/ui/dropdown-menu/DropdownMenuRadioGroup.vue';

const meta = {
  title: 'DropdownMenuRadioGroup',
  component: DropdownMenuRadioGroup,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenuRadioGroup>;

export default meta;
type Story = StoryObj<typeof DropdownMenuRadioGroup>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};