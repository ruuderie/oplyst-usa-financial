import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarRadioGroup from '../components/ui/menubar/MenubarRadioGroup.vue';

const meta = {
  title: 'MenubarRadioGroup',
  component: MenubarRadioGroup,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarRadioGroup>;

export default meta;
type Story = StoryObj<typeof MenubarRadioGroup>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};